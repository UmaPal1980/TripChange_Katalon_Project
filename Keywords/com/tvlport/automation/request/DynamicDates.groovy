package com.tvlport.automation.request

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

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
import java.text.SimpleDateFormat
import java.util.concurrent.ThreadLocalRandom
class DynamicDates {

	@Keyword
	def  getDynamicDate(String dateModifier) {
		//String dateModifier=GlobalVariable.term_DynamicDate
		int iteneraryValue=Integer.parseInt(dateModifier.substring(4));
		SimpleDateFormat dateFormat = new SimpleDateFormat("ddMMM");
		String date=null;
		System.out.println("split\t"+dateModifier.substring(4));
		if(dateModifier.equals("date0")) {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.DATE, 5);
			Date d = cal.getTime();
			Calendar cal1 = new GregorianCalendar();
			cal1.add(Calendar.DATE, 14);
			Date d1 = cal1.getTime();

			System.out.println(d.toString()+"\t"+(d1.toString()));
			long random = ThreadLocalRandom.current().nextLong(d.getTime(), d1.getTime());
			Date dateValue = new Date(random);

			date = dateFormat.format(dateValue).toUpperCase();
		}
		else{
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.DATE, 25*iteneraryValue+4);
			Date d = cal.getTime();

			Calendar cal1 = new GregorianCalendar();
			cal1.add(Calendar.DATE, 25*iteneraryValue+13);
			Date d1 = cal1.getTime();
			System.out.println(d.toString()+"\t"+(d1.toString()));
			long random = ThreadLocalRandom.current().nextLong(d.getTime(), d1.getTime());
			Date dateValue = new Date(random);
			date = dateFormat.format(dateValue).toUpperCase();
		}
		return date;
	}

	@Keyword
	def  getDate_TSSearch(String dateModifier) {
		//String dateModifier=GlobalVariable.term_DynamicDate
		int iteneraryValue=Integer.parseInt(dateModifier.substring(4));
		SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd");
		String date=null;

		System.out.println("split\t"+dateModifier.substring(4));
		if(dateModifier.equals("date0")) {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.DATE, 5);
			Date d = cal.getTime();

			Calendar cal1 = new GregorianCalendar();
			cal1.add(Calendar.DATE, 14);
			Date d1 = cal1.getTime();

			System.out.println(d.toString()+"\t"+(d1.toString()));
			long random = ThreadLocalRandom.current().nextLong(d.getTime(), d1.getTime());
			Date dateValue = new Date(random);

			date = dateFormat.format(dateValue);
		}
		else{
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.DATE, 25*iteneraryValue+4);
			Date d = cal.getTime();

			Calendar cal1 = new GregorianCalendar();
			cal1.add(Calendar.DATE, 25*iteneraryValue+13);
			Date d1 = cal1.getTime();
			System.out.println(d.toString()+"\t"+(d1.toString()));
			long random = ThreadLocalRandom.current().nextLong(d.getTime(), d1.getTime());
			Date dateValue = new Date(random);
			date = dateFormat.format(dateValue);
		}
		return date;
	}
	@Keyword
	def  getBirthDate(String ptc) {
		//String dateModifier=GlobalVariable.term_DynamicDate
		SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd");
		String date=null;

		if(ptc.equalsIgnoreCase("INS")||ptc.equalsIgnoreCase("INF")) {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.MONTH, -9);
			Date d = cal.getTime();
			date = dateFormat.format(d);
		}
		else if(ptc.equalsIgnoreCase("CHD")||ptc.equalsIgnoreCase("CNN")){
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.YEAR, -9);
			Date d = cal.getTime();
			date = dateFormat.format(d);
		}
		else {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.YEAR, -19);
			Date d = cal.getTime();
			date = dateFormat.format(d);
		}
		return date;
	}

	@Keyword
	def  getPassportDate(String issueExpiryDate) {
		//String dateModifier=GlobalVariable.term_DynamicDate
		SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd");
		String date=null;

		if(issueExpiryDate.contains("Issue")) {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.MONTH, -6);
			Date d = cal.getTime();
			date = dateFormat.format(d);
		}
		else {
			Calendar cal = new GregorianCalendar();
			cal.add(Calendar.YEAR,10);
			Date d = cal.getTime();
			date = dateFormat.format(d);
		}
		return date;
	}
}