import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testcase.TestCaseFactory.*
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext
import groovy.io.FileType

import org.codehaus.groovy.scriptom.ActiveXObject;
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords

import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import static groovy.io.FileType.FILES

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import static org.assertj.core.api.Assertions.*


import com.kms.katalon.core.webservice.verification.WSResponseManager



import org.apache.poi.xssf.usermodel.XSSFSheet;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;
import org.apache.poi.xssf.*;
import org.apache.poi.hssf.util.HSSFColor
import org.apache.poi.ss.usermodel.*;


class DataInfo {
	/**
	 * Executes before every test case starts.
	 * @param testCaseContext related information of the executed test case.
	 */
	@BeforeTestCase
	def sampleBeforeTestCase(TestCaseContext testCaseContext) {
		//println testCaseContext.getTestCaseId()
		//println testCaseContext.getTestCaseVariables()

		GlobalVariable.sEnvironment = findTestData('Data Files/Environment').getValue('ExecuteOn', 1)
		println ('The environment to Run the test is '+ GlobalVariable.sEnvironment)
		def sEnvironmentCol

		if(GlobalVariable.sEnvironment == "QAG")

		{


			sEnvironmentCol = 3

		}

		else if (GlobalVariable.sEnvironment == "INT")

		{
			sEnvironmentCol = 4

		}

		else if (GlobalVariable.sEnvironment == "DEV")

		{

			sEnvironmentCol = 5

		}

		GlobalVariable.sSearchURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol , 1)
		//println (GlobalVariable.sSearchURL)
		GlobalVariable.sPriceURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 2)
		//println (GlobalVariable.sPriceURL)
		GlobalVariable.sInitiateWBURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 3)
		//println (GlobalVariable.sInitiateWBURL)
		GlobalVariable.sAddofferCatalogueURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 4)
		//println (GlobalVariable.sAddofferCatalogueURL)
		GlobalVariable.sAddTravellerURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 5)
		//println (GlobalVariable.sAddTravellerURL)
		GlobalVariable.sCommitURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 6)
		//println (GlobalVariable.sCommitURL)
		GlobalVariable.sInitiateWBwithPNRURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 7)
		//println (GlobalVariable.sInitiateWBwithPNRURL)
		GlobalVariable.sAddFOPURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 8)
		//println (GlobalVariable.sAddFOPURL)
		GlobalVariable.sAddPaymentURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 9)
		//println (GlobalVariable.sAddPaymentURL)
		GlobalVariable.sTicketIssuanceURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 10)
		//println (GlobalVariable.sTicketIssuanceURL)
		GlobalVariable.sGetEligibilityURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 11)
		//println (GlobalVariable.sGetEligibilityURL)
		GlobalVariable.sReservationWBURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 12)
		//println (GlobalVariable.sReservationWBURL)
		GlobalVariable.sSearchDomainPNRURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 13)
		//println (GlobalVariable.sSearchDomainPNRURL)
		GlobalVariable.sModifyOfferURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 14)
		//println (GlobalVariable.sModifyOfferURL)
		GlobalVariable.sAddPaymentExchURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 15)
		//println (GlobalVariable.sAddPaymentExchURL)
		GlobalVariable.sReservationCommitURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 16)
		//println (GlobalVariable.sReservationCommitURL)
		GlobalVariable.sHCA_URL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 17)

		GlobalVariable.sGetEligibilitybyTKTURL = findTestData('Data Files/Environment').getValue(sEnvironmentCol, 18)


		def TestDataFile = findTestData('Data Files/TestData')
		def sTestCaseID = testCaseContext.getTestCaseId()
		sTestCaseID = sTestCaseID.trim()
		sTestCaseID = sTestCaseID.substring((sTestCaseID.lastIndexOf("/").toInteger()) + 1)
		sTestCaseID = sTestCaseID.toUpperCase()


		def sTestCaseFolder = testCaseContext.getTestCaseId()
		//sTestCaseFolder = sTestCaseFolder.indexOf("/", 3)
		String[] aTestCaseFolder  = sTestCaseFolder.split("/")
		GlobalVariable.sComponentLevelTest = aTestCaseFolder[1]
		GlobalVariable.sComponentTestType = aTestCaseFolder[2]
		//println ('InputFileName are ' + GlobalVariable.sComponentLevelTest)

		//println ('InputFileName are ' + GlobalVariable.sComponentTestType)

		//def sTestCaseName = findTestData('Data Files/TestData').getValue('TCName', 1)

		int sColumnNumber = findTestData('Data Files/TestData').columnNumbers
		//println ('TotalColumn are ' + sColumnNumber)

		int srowNumbers = findTestData('Data Files/TestData').rowNumbers
		//println ('TotalRows are ' + srowNumbers)

		//Loop to Iterate Rows

		for (int iRowindex = 1; iRowindex < srowNumbers+1; iRowindex++){

			//Loop to Iterate Column

			String sTestCaseName = TestDataFile.getObjectValue('TCName', iRowindex)

			sTestCaseName = sTestCaseName.trim()

			sTestCaseName = sTestCaseName.toUpperCase()

			GlobalVariable.sTestCaseName = sTestCaseName

			//println ('Testcase ID is ' + sTestCaseID + ' & sTestCaseName is ' + sTestCaseName)



			if(sTestCaseID == sTestCaseName)

				//println ('Testcase ID is ' + sTestCaseID + 'sTestCaseName is ' + sTestCaseName)


			{

				//for (int iColumnindex = 1; iColumnindex < sColumnNumber; iColumnindex++)


				//{


				GlobalVariable.sPCC = TestDataFile.getValue('PCC', iRowindex)
				GlobalVariable.sFrom = TestDataFile.getValue('From', iRowindex)
				GlobalVariable.sTo = TestDataFile.getValue('To', iRowindex)
				GlobalVariable.sCircleTripApt = TestDataFile.getValue('CircleTripApt', iRowindex)
				GlobalVariable.sCarrier = TestDataFile.getValue('Flight', iRowindex)
				def sdepartureDate = TestDataFile.getValue('departureDate', iRowindex)
				def sRetnDate = TestDataFile.getValue('RetnDate', iRowindex)
				GlobalVariable.sPTC = TestDataFile.getValue('passengerTypeCode', iRowindex)
				GlobalVariable.sPassengerCount = TestDataFile.getValue('PassengerCount', iRowindex)
				GlobalVariable.sCurrencyCode = TestDataFile.getValue('CurrencyCode', iRowindex)
				GlobalVariable.sFormOfPayment = TestDataFile.getValue('FormofPayment', iRowindex)
				GlobalVariable.sFileName = TestDataFile.getValue('FileName', iRowindex)
				GlobalVariable.sExecutionFlow = TestDataFile.getValue('ExecutionFlow', iRowindex)
				GlobalVariable.sClassOfService = TestDataFile.getValue('ClassOfService', iRowindex)
				GlobalVariable.FareEntry = TestDataFile.getValue('FareEntry', iRowindex)




				//-------------------------------------------------------------------------------------
				GlobalVariable.sExch_From = TestDataFile.getValue('Exch_From', iRowindex)
				GlobalVariable.sExch_To = TestDataFile.getValue('Exch_To', iRowindex)
				GlobalVariable.sExch_Flight = TestDataFile.getValue('Exch_Flight', iRowindex)
				def sExch_departureDate = TestDataFile.getValue('Exch_departureDate', iRowindex)
				def sExch_RetnDate = TestDataFile.getValue('Exch_RetnDate', iRowindex)
				GlobalVariable.sExchDepDay = sExch_departureDate
				GlobalVariable.sExchRetDay = sExch_RetnDate
				GlobalVariable.sTestCaseID = sTestCaseID

				//GlobalVariable.sCurrencyCode = TestDataFile.getValue('CurrencyCode', iColumnindex)

				KeywordUtil.logInfo('----The Katalon Test Case Name is ' + sTestCaseID)
				KeywordUtil.logInfo('----The TestData Test Case Name is ' + sTestCaseName)


				//println('The row no. is  ' + (iRowindex+1))
				//println('The Katalon Test Case Name is ' + sTestCaseID)
				//println('the Excel Test Case Name is ' + sTestCaseName)

				//println('The PCC is  ' + (sPCC))
				//println('The From City is ' + GlobalVariable.sFrom)
				//println('The TO City is ' + GlobalVariable.sTo )
				//println('The Carrier Code is  ' + GlobalVariable.sCarrier)
				//println('The PTC Code is  ' + GlobalVariable.sPTC)
				//println('The Passenger count is ' + GlobalVariable.sPassengerCount)
				//println('The Currency Code is ' + GlobalVariable.sCurrencyCode)

				//---------------------------------------------------------------------------------------
				//println('The Exchange From City is  ' + GlobalVariable.sExch_From)
				//println('the Exchange TO City is ' + GlobalVariable.sExch_To)
				//	println('the Exchange Flight is ' + GlobalVariable.sExch_Flight)


				if(sdepartureDate.length() > 0)

				{


					int sDays = sdepartureDate as int
					def today = new Date()
					def sDate = today + sDays

					if (GlobalVariable.sExecutionFlow == 'TerminalPNR') {
						GlobalVariable.sDepDate = sDate.format("ddMMM")
						println('the departure is ' + GlobalVariable.sDepDate)

					}else {

						GlobalVariable.sDepDate = sDate.format("yyyy-MM-dd")
						println('the departure is ' + GlobalVariable.sDepDate)

					}
				}

				//------------------------------------------------------------------

				if(sdepartureDate.length() > 0)
				{
					int sCircleDays = sdepartureDate as int
					def today = new Date()
					def sCircleDate = today + sCircleDays + 3

					if (GlobalVariable.sExecutionFlow == 'TerminalPNR') {
						
						GlobalVariable.sCircleDate = sCircleDate.format("ddMMM")
						println('the CircleDate is ' + GlobalVariable.sCircleDate)

					}else {
						GlobalVariable.sCircleDate = sCircleDate.format("yyyy-MM-dd")
						println('the CircleDate is ' + GlobalVariable.sCircleDate)

					}

				}
				//------------------------------------------------------------------

				if(sRetnDate.length() > 0)
				{
					int sRetDays = sRetnDate as int
					def today = new Date()
					def sReturnDate = today + sRetDays
					if (GlobalVariable.sExecutionFlow == 'TerminalPNR') {

						GlobalVariable.sRetDate = sReturnDate.format("ddMMM")
						println('the Return Date is ' + GlobalVariable.sRetDate)

					}else {
						GlobalVariable.sRetDate = sReturnDate.format("yyyy-MM-dd")
						println('the Return Date is ' + GlobalVariable.sRetDate)

					}
				}
				//------------------------------------------------------------------

				if(sExch_departureDate.length() > 0)
				{
					int sExch_DepartureDays = sExch_departureDate as int
					def today = new Date()
					def sExchDep = today + sExch_DepartureDays

					GlobalVariable.sExch_departureDate = sExchDep.format("yyyy-MM-dd")
					println('the Exchange Departure Date is ' + GlobalVariable.sExch_departureDate)
				}

				//------------------------------------------------------------------

				if(sExch_RetnDate.length() > 0)
				{
					int sExch_RetDays = sExch_RetnDate as int
					def today = new Date()
					def sExchReturnDate = today + sExch_RetDays
					GlobalVariable.sExch_RetnDate = sExchReturnDate.format("yyyy-MM-dd")
					println('the Exchange Returned Date is ' + GlobalVariable.sExch_RetnDate)
					//------------------------------------------------------------------
				}





				def sAccessGroup

				if(GlobalVariable.sPCC == "O1P")

				{

					GlobalVariable.sCustomerProfileId = "ODTS_1G_O1P_9FA9B7"
					println('The ProfileID is '+ GlobalVariable.sCustomerProfileId)
					sAccessGroup = "19CF434B-3D98-4312-B626-09E427FC1691"
					GlobalVariable.sAccessGroup = sAccessGroup
					println('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)

				}

				else if (GlobalVariable.sPCC == "79JP")

				{
					GlobalVariable.sCustomerProfileId = "ODTS_1G_79JP_9FA7E1"
					GlobalVariable.sAgencyCity = "SYD"
					println('The ProfileID is '+ GlobalVariable.sCustomerProfileId)
					sAccessGroup = "7C7ED10A-EEBC-4468-B499-879DE63F1B7D"
					GlobalVariable.sAccessGroup = sAccessGroup
					println('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)

				}

				else if (GlobalVariable.sPCC == "62HA")

				{
					GlobalVariable.sCustomerProfileId = "ODTS_1G_62HA_9FA9AE"
					println('The ProfileID is '+ GlobalVariable.sCustomerProfileId)
					sAccessGroup = "F976DEE8-BB4C-4912-83AA-B3D89BDC04C7"
					GlobalVariable.sAccessGroup = sAccessGroup
					println('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)

				}

				else if (GlobalVariable.sPCC == "51MG")

				{

					GlobalVariable.sCustomerProfileId = "ODTS_1G_51MG_9FADBA"
					println('The ProfileID is '+ GlobalVariable.sCustomerProfileId)
					sAccessGroup = "898C55E6-0DD6-4B4D-8D56-0B96033CB150"
					GlobalVariable.sAccessGroup = sAccessGroup
					println('The PCC "' + GlobalVariable.sPCC +'" AccessGroup ID is ' + GlobalVariable.sAccessGroup)

				}



				break;

				//}




			}





		}






	}






	/**
	 * Executes after every test case ends.
	 * @param testCaseContext related information of the executed test case.
	 */
	@AfterTestCase
	def sampleAfterTestCase(TestCaseContext testCaseContext) {
		//println testCaseContext.getTestCaseId()
		//println testCaseContext.getTestCaseStatus()

		if(testCaseContext.getTestCaseStatus()== 'PASSED'){

			System.out.println("Test case is Passed")


		}

		else if(testCaseContext.getTestCaseStatus()== 'Failed')

		{
			System.out.println("Test case is Failed")


		}



	}

	/**
	 * Executes before every test suite starts.
	 * @param testSuiteContext: related information of the executed test suite.
	 */
	@BeforeTestSuite
	def sampleBeforeTestSuite(TestSuiteContext testSuiteContext) {

		println testSuiteContext.getTestSuiteId()



	}

	/**
	 * Executes after every test suite ends.
	 * @param testSuiteContext: related information of the executed test suite.
	 */
	@AfterTestSuite
	def sampleAfterTestSuite(TestSuiteContext testSuiteContext) {
		println testSuiteContext.getTestSuiteId()

		if(testSuiteContext.getStatus()== 'PASSED'){

			//System.out.println("Test Suite is Passed")
			KeywordUtil.markPassed("Test Suite is Passed")



		}

		else if(testSuiteContext.getStatus()== 'Failed') {

			//System.out.println("Test Suite is Failed")
			KeywordUtil.markFailed("Test Suite is Failed, due to one or more test cases are failed")


		}


	}


}